The printed output already looks right. The library test calls the helper separately.
format! builds text as a String. The starter's println! displays text immediately.
Move the decoration into the helper and pass its result to the binary's println!.
