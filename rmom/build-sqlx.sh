#!/bin/bash

source environment

cargo sqlx prepare -- --lib
