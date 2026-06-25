 #include<iostream>
 using namespace std;
 void TwoSum(int num[],int n,int target){
    
    for(int i=0;i<n-1;i++){
        for(int j=i+1;j<n;j++){
            if(num[i]+num[j]==target){
                cout<<'['<<i<<','<<j<<']'<<endl;
                return;
            }
        }
    }
 }
 int main(){
    int num[]={1,2,8,9,6,45,32,12,11};
    int n = sizeof(num)/sizeof(num[0]);
    TwoSum(num,n,14);
    return 0;
 }