    #[test]
    fn test_pathbuf_to_foundfile() {
        use std::path::PathBuf;
        use list::FoundFile;

         let file: FoundFile = PathBuf::from("./foo.txt").into(); 

         assert_eq!(file.name, "foo.txt");
    }
