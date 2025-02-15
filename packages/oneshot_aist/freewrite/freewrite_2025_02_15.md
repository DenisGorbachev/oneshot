# Freewrite on 2025-02-15

* I need a signature for `count_files_in_dir` function
  * I need a return type
    * Notes
      * I don't know how many files can be present in a single directory, so I don't know how to select a specific return type
      * I know that return type must be an unsigned integer, but I don't know how many bits to use, because I don't know how many files a directory may contain
        * If the function is recursive, the count doesn't have an upper bound
        * If the function is non-recursive (counts only in the current dir), then the count upper bound is equal to the max count of entries for a dir
          * The max count for entries in a dir depends on a file system
            * I can use the max of max counts for a specific list of file systems
    * Options
      * Look up the max number of files in dir
      * Use an Option<u64> to model overflow
