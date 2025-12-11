# give me a summary of the entirety of the glia project for a new chat to understand what it has become thus far been taken thus far, in a copyable markdown code block
# Philosophy
- [ ] What am I actually trying to solve?
    - I want to gauge how much my tool is being used, to see how much adoption it has vs other alternatives
    - I want to have ways of doing benchmarking passively, without having to request people doing to submit their stats
- [ ] What do people actually care about?
    ## The User
    - Their gains vs previous program
    - Not being slowed down by engineerial details
    ## The Dev / Admin
    - Ensuring people are using the new tools
    - That new tool is actually more efficient
    - Ways to optimise their tool
- [ ] The max scope of v0
    - 2 clients: Python and R
        - Collection of data - single script model
    - 1 way of pushing data to database
    - 1 database
    - 1 way of getting a JSON dump
- [ ] The max scope of v0
    - 1 Nextflow client
- [ ] How long do I expect it to take?
    - Max 2 days: Wednesday the 10th, Thursday the 11th


# Setup
-- [x] Lock files
    
- [x] A task runner
- [x] CCs 
- [x] Basic dir structure
- [] Dependencies
    - General dependencies:
        - Docker
        - [x] mise
        - [x] R
        - [x] Python
            - [x] FastAPI
    - dev dependencies
        - [x] gum
- [ ] Docs
    - [ ] Basic README
- [ ] Identify testing
- [ ] Git
    - [ ] Merge branches protections, only stiff I approve can be merged
    - [ ] merged branches get deleted


# Clients
- [ ] Architecture - use
    - [ ] How will it actually be implemented in R?
    - [ ] How will it actually be implemented in Python?    

# Backend
- [ ] Data Schema
    - [x] Choose compulsory variables
        - run_id
        - program_name
        - user_name
        - script_sha256
        - exit_code_int
        - started_at
        - ended_at
        - cpu_time_sec
        - cpu_percent
        - max_rss_mb
    - 