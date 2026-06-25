 #include<iostream>
 #include<unordered_set>
 using namespace std;
 bool is_contains_duplicate(int num[],int n){
    unordered_set<int> hashset;
    for(int i=0;i<n;i++){
        hashset.insert(num[i]);
    }
    if(hashset.size()==n) return false;
    return true;

    
    
 }
 int main(){
    int num[]={1,2,8,9,6,45,32,12,11};
    int n = sizeof(num)/sizeof(num[0]);
    cout<<is_contains_duplicate(num,n)<<endl;
    
    return 0;
 }