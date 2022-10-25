use std::collections::HashMap;

/// `InputCellId` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct InputCellId(usize);

/// `ComputeCellId` is a unique identifier for a compute cell.
/// Values of type `InputCellId` and `ComputeCellId` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellId = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellId = r.create_compute(&[react::CellId::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackId(usize);

/// `CellId` is a unique identifier for a cell.
/// Cell can be an input cell or compute cell
#[derive(Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

/// Callback Remove operation errors
#[derive(Debug, PartialEq, Eq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type BoxFn<'cf, T> = Box<dyn Fn(&[T]) -> T + 'cf>;
type Callback<'cb, T> = (CallbackId, Box<dyn FnMut(T) + 'cb>);

/// definition of a cell. It can be an input cell or compute cell
struct ReactorCell<'cb, 'cf, T> {
    id: CellId,
    value: Option<T>,
    dependencies: Vec<CellId>,
    compute_func: Option<BoxFn<'cf, T>>,
    callbacks: Vec<Callback<'cb, T>>,
}

impl<'cb, 'cf, T: Copy> ReactorCell<'cb, 'cf, T> {
    // creates an input cell
    fn create_input_cell(id: InputCellId, value: T) -> Self {
        Self {
            id: CellId::Input(id),
            value: Some(value),
            dependencies: vec![],
            compute_func: None,
            callbacks: vec![],
        }
    }

    // creates an compute cell
    // value is initialized as None
    fn create_compute_cell(
        id: ComputeCellId,
        dependencies: &[CellId],
        compute_func: BoxFn<'cf, T>,
    ) -> Self {
        Self {
            id: CellId::Compute(id),
            value: None,
            dependencies: dependencies.to_vec(),
            compute_func: Some(compute_func),
            callbacks: vec![],
        }
    }

    // get value of a cell
    fn get_value(&self) -> Option<T> {
        self.value
    }

    // set value of a cell
    fn set_value(&mut self, value: T) {
        self.value = Some(value);
    }
}

pub struct Reactor<'cb, 'cf, T> {
    cells: Vec<ReactorCell<'cb, 'cf, T>>,
    dependecies_map: HashMap<CellId, Vec<CellId>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'cb, 'cf, T: Copy + PartialEq> Reactor<'cb, 'cf, T> {
    pub fn new() -> Self {
        Self {
            cells: vec![],
            dependecies_map: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    // value of the cell is set to the initial value provided
    pub fn create_input(&mut self, initial: T) -> InputCellId {
        let id = self.cells.len() + 1;
        let id = InputCellId(id);
        let cell = ReactorCell::create_input_cell(id, initial);
        self.cells.push(cell);
        id
    }

    // Creates a compute cell with the provided dependecies and compute_func
    // after the cell is created, compute is called on the cell to set the initial value of the cell
    pub fn create_compute<F: Fn(&[T]) -> T + 'cf>(
        &mut self,
        dependencies: &[CellId],
        compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        let id = self.cells.len() + 1;
        let id = ComputeCellId(id);
        // if any of the provided dependencies are invalid then return error
        for cell in dependencies {
            if !self.is_valid_cell(*cell) {
                return Err(*cell);
            }
        }
        // create the compute cell
        let cell = ReactorCell::create_compute_cell(id, dependencies, Box::new(compute_func));
        self.cells.push(cell);
        // add the ComputeCellId to the dependencies_map
        for cell in dependencies {
            self.dependecies_map
                .entry(*cell)
                .and_modify(|v| v.push(CellId::Compute(id)))
                .or_insert_with(|| vec![CellId::Compute(id)]);
        }
        // compute value for the cell just created
        self.compute(CellId::Compute(id));
        Ok(id)
    }

    // check if a given CellId exists
    fn is_valid_cell(&self, id: CellId) -> bool {
        self.cells.iter().any(|c| c.id == id)
    }

    // compute value for a given CellId
    fn compute(&mut self, id: CellId) {
        // compute not for InputCell
        if let CellId::Input(_) = id {
            return;
        }
        let deps = self.get_dependencies_values(id);
        let cell = self.cells.iter_mut().find(|cell| cell.id == id).unwrap();
        // compute cell must have a compute_func, hence unwrap
        let val = (cell.compute_func.as_ref().unwrap())(&deps);
        // keep the old_val before updating
        let old_val = cell.get_value();
        cell.set_value(val);
        // if new value has changed then call all added callbacks
        if old_val.is_none() || old_val.unwrap() != val {
            for cb in cell.callbacks.iter_mut() {
                (cb.1)(val);
            }
        }
    }

    // get values of cells that are dependencies of a cell
    fn get_dependencies_values(&self, id: CellId) -> Vec<T> {
        self.cells
            .iter()
            .find(|cell| cell.id == id)
            .unwrap() // well cell must exists to have dendencies
            .dependencies
            .iter()
            .map(|c| self.value(*c).unwrap())
            .collect::<Vec<_>>()
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    pub fn value(&self, id: CellId) -> Option<T> {
        let cell = self.cells.iter().find(|c| c.id == id)?;
        cell.get_value()
    }

    // Sets the value of the specified input cell.
    // Returns false if the cell does not exist.
    pub fn set_value(&mut self, id: InputCellId, new_value: T) -> bool {
        let cell = self.cells.iter_mut().find(|c| c.id == CellId::Input(id));
        // if cell doesn't exists then return false
        if cell.is_none(){
            return false;
        }
        let cell = cell.unwrap();
        cell.set_value(new_value);
        let dependants = self.get_all_dependants(id);
        for dep in dependants {
            self.compute(dep);
        }
        true
    }

    // get all dependants of a InputCellId 
    // get all dependants of all dependants
    fn get_all_dependants(&self, id: InputCellId) -> Vec<CellId> {
        let dependants = self.dependecies_map.get(&CellId::Input(id));
        // if no dependants present then return
        if dependants.is_none() {
            return vec![];
        }
        let mut dependants = dependants.unwrap().clone();
        let mut i = 0;
        loop {
            if i >= dependants.len() {
                break;
            }
            if let Some(deps) = self.dependecies_map.get(&dependants[i]) {
                for dep in deps {
                    if !dependants.contains(dep) {
                        dependants.push(*dep);
                    }
                }
            }
            i += 1;
        }
        dependants.sort_unstable();
        dependants
    }

    // Adds a callback to the specified compute cell.
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    pub fn add_callback<F: FnMut(T) + 'cb>(
        &mut self,
        id: ComputeCellId,
        callback: F,
    ) -> Option<CallbackId> {
        if let Some(cell) = self.cells.iter_mut().find(|c| c.id == CellId::Compute(id)) {
            let cb = cell.callbacks.len() + 1;
            let cb = CallbackId(cb);
            cell.callbacks.push((cb, Box::new(callback)));
            return Some(cb);
        }

        None
    }

    // Removes the specified callback, using an ID returned from add_callback.
    // Returns an Err if either the cell or callback does not exist.
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let cell = self
            .cells
            .iter_mut()
            .find(|c| c.id == CellId::Compute(cell))
            .ok_or(RemoveCallbackError::NonexistentCell)?;
        let cb_idx = cell
            .callbacks
            .iter_mut()
            .position(|c| c.0 == callback)
            .ok_or(RemoveCallbackError::NonexistentCallback)?;
        let _ = cell.callbacks.remove(cb_idx);
        Ok(())
    }
}

// implement Default trait for Reactor and make Clippy happy :-)
impl<'cb, 'cf, T: Copy + PartialEq> Default for Reactor<'cb, 'cf, T> {
    fn default() -> Self {
        Self::new()
    }
}
