# waterlevel

## IOT Portion

waterlevel is a solution to measuring and keeping track of the water level, or gauge height, of a pool. It measures the level of the pool using an ultrasonic distance sensor and Arduino microcontroller mounted to the lid of a pool skimmer or anywhere directly above, and reasonably close to, the water. It should ideally be located in a location with very few waves as that could induce error in the sensor's measurements.

## API Portion

waterlevel not only measures the height of the water at regular intervals, but also stores these measurements in a PostgreSQL database and provides a backend API to fetch processed or raw data that could be used for something like an IOT controlled water hose to refill the pool if water levels are low.

This will likely become either a part of this project or its own project to be tackled after this one.

## AWS Integration

This project will integrate with the AWS cloud for database and backend hosting using CloudFormation IAC, but will always provide the ability to run its components either in docker containers or locally via compiling the source code.

## Rust

This project will be written entirely in the Rust programming language as I'm using it as a means to grow my skill with it and IAC cloud architecture.
