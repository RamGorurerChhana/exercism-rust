function isPalindrome(a){
    a = a.toString().split('');
  
    for(var i=0; i<(a.length/2); i++){
      if(a[i] !== a[a.length -1 -i]){
        return false;
      }
    }
  
    return true;
  }
  
  
  function largestPalindrome(){
    let ans = 0;
    for(let x = 999; x >= 100; x--){
      for(let y = 999; y>= 100; y--){
        let z = x * y;
        if (z < ans) {
          break;
        }
        if(isPalindrome(z) && z > ans){
          ans = z;
        }
      }
    }
    return ans;
  }
  
  console.log(largestPalindrome())
  
  