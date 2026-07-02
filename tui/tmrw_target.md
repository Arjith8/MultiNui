# This page will contain the requirements i intend to statisfy the next day

- we would have a simple layout similr to fzf but inverter 
    - Create 2 sections in the page 
        - Top will will have a max limit on size/will use same len in all screen sizes
        - this will be the tabs section the active tab will have diff color u know what to do
        - The tabs ig will be in a sqlite db Tabs Table we will keep track of deleted creation etc
            - Tabs Table Structure
                - id: autoincrement int
                - name: string
                - created_at: datetime
                - deleted_at: datetime

            - Tab name editting doesnt retain much info in the sense that we dont track it
        - To get these tabs we need to use some sqlite db connector
            - this will be in the FE as we do not intent to have multiple users accessing 
            - The syncing is done using turo the user will be required to provide API key for it

        - Another table we will need is updates
            - This should act a bit like WAL in postgres
                - we need to have an idea on how many devices are connected if not wal will be cleared too early
                    - If wal is cleared we will mark it in the device table let the user know that entries in wal say exceeded 1000 and we have now marked the devices as inactive this would mean we will perform a full remote db copy
            - For a new app instance as in user signs up to say the tui for the first time we will copy and run migrations as necessary
            - Since we dont have any ORM to use we will follow similar str to 


# Data syncing
- The more complex part will be the syncing between multiple devices but for now we can assume that one person will only have updates on one device
- The conflicts can arrise in a few way
    1. they update diff things in that case we just update ids and and conbine
    2. update on same table/entries 
        - Then we will have to evaluate stratergy based on what sorta update it is
            - Say the user created an entry from device 1 and removed it from device 2 before sync(ahhh this wont happen btw)
