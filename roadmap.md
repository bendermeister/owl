# Roadmap

## TF-IDF
- [ ] File Indexer
  - [ ] discover all files in a given directory 
  - [ ] filter all files which not have been touched since last dicovery
  - [ ] apply a function to all files 
  - [ ] update files in database
- [ ] TF-IDF
  - General Description:
    **TF(t: Word, d: Document)**: d.count(t) / d.len()
    **IDF(t: Word, d: &[Document])**: log(d.len() / d.iter().filter(|d| d.contains(t)).count())
  - [X] DB Schema
    ```sql
        CREATE TABLE files (
            id INTEGER NOT NULL UNIQUE,     -- id of the given file
            path TEXT NOT NULL UNIQUE,      -- path of the given file
            last_touched INTEGER,           -- timestamp of last recorded change 
                                            -- to the file -> can be compared to
                                            -- last change of file to check if 
                                            -- file has changed in since the last
                                            -- discovery
            PRIMARY KEY(id)
        );

        CREATE TABLE terms (
            id      INTEGER NOT NULL UNIQUE,
            term    TEXT NOT NULL UNIQUE,
            idf     REAL NOT NULL,

            PRIMARY KEY(id)
        );

        CREATE TABLE term_frequencies (
            term    INTEGER NOT NULL,
            file    INTEGER NOT NULL,
            tf      REAL    NOT NULL,

            FOREIGN KEY(term) REFERENCES terms(id),
            FOREIGN KEY(file) REFERENCES files(id)
        );
    ```
