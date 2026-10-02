#include <iostream>
#include <unordered_map>

using name = std::string;
using age = int;
using person = std::pair<name, age>;
using address = std::string;
using address_book = std::unordered_map<person, address>;

void print_address_book(const address_book &book)
{
    for (const auto &[person, address] : book)
    {
        std::cout << person.first << " is " << person.second << " years old and lives at " << address << std::endl;
    }
}

int main()
{

    address_book people{};
    people.insert({{"John", 20}, "221B Baker Street, London"});
    people.insert({{"Mary", 30}, "Avenue des Champs-Élysées, Paris"});
    people.insert({{"Jack", 73}, "Wall Street, New York"});
    print_address_book(people);

    return 0;
}
