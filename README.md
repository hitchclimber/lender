# Lender

This is a simple REST API that allows users to lend and borrow media in their region. It's mainly for educational/training purposes (`actix_web`, `sqlx`), since this would require a full stack. Maybe I'll add a front end at some point, but for actual use it's pretty pointless, since it would require infrastructure and users. To my knowledge, there has been a similar project which failed/never got their tracks on. I guess one problem is that most people use ebay for this kind of thing anyway. As I said, this is mainly for training/fun!

## TODO

- [x] Add basic CRUD operations for users
- [x] Add basic CRUD operations for media
- [ ] Add localisation
     &rarr; tried extensively to make it work, but had to use a simple `String` for cities as a workaround until I find a better solution or `sqlx` plays better with geometries out of the box


