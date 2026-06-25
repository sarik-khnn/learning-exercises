#include<iostream>
using namespace std;
int main(){
    int n=5;
    int star = 1;

    for(int i=1;i<(2*n);i++){
        
        for(int k=1;k<=star;k++){
            cout<<'*';
        }cout<<endl;
        if(i<n){
            
            star++;
        }else{
            
            star--;
        }


    }
        
}