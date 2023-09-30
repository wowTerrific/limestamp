# Limestamp

🚧 In progress 🚧


### Important
First version will not account for Daylight Savings Time... This will be implemented in the future.

## TODO
- Parse offset
    - unit tests for helper functions
    - idomatic way to set the return message for all functions
- Start work on `limestamp()`
- Create Options struct for `limestamp` to take as an argument
    - implement a builder
    - reference an Enum/table of all the timezones so that a user only needs to input that instead of a custom Offset number
    - The timezone table will also hold information regarding daylight savings time for those zones
        - I don't know how to maintain this if things change other than referencing a website on occasion
