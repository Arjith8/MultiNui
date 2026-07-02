what all things do i want
- Base level just a way to add store and persist my tasks
    - Includes CRUD
        - Add task
        - remove
        - update
            - status update
            - rename/provide new description change frequency etc

Technical Info
- From what I can see in the docs for applications atte we will need
    - Two DB urls? i mean i just need the state so ig thats it
        - why 2 one will be remote and synced which is wrt turso other a local sqlite db so
            - My goal is to only periodically check and update db as necessary
    - `exit` which seems to be how ratatui knows if the user has initiated quit or nah



