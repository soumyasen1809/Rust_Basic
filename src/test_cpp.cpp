#include <iostream>

int main(){
    int count = 0;
    while (true)
    {
        count++;
        std::cout << "Start count: " << count << std::endl;

        if (count == 3){
            std::cout << "Inside three loop" << std::endl;
            continue;
        }

        std::cout << "After three loop with count: " << count << std::endl;

        if (count == 6){
            std::cout << "Inside five loop" << std::endl;
            break;
        }

        std::cout << "End count: " << count << std::endl;

        while (true)
        {
            count++;
            std::cout << "Inside 2nd loop with count: " << count << std::endl;
            break;
        }
        
        std::cout << "End 2nd while with count: " << count << std::endl;
    }
    
    


    return 0;
}