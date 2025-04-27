# Quote Server
## Creator: Sarah Dylan

## Project Description
The goal of this project so far is to create a full stack website that can serve quotes. I am utilizing Axum as the web framework, askama for creating templates, serde for serialization, sqlite for database needs, and SeaOrm for object relational mapping. 

In class, we are using sqlx, but I wanted to get more experience using an ORM or to get a feel for how ORM's work in Rust. I have limited experience using Django and SqlAlchemy/Alembic in Python projects so it was very interesting comparing my previous experiences to my time with SeaOrm. So far, I've been pretty impressed with how straightforward SeaOrm feels. It was really easy to get a QuoteModel all the way from the database to the askama template. The main thing that I do find odd was the autogeneration of entity classes. Although it is an option provided by the sea-orm-cli, I still had to go in and manually edit the file so it would allow me to not include an ID when serializing.

Most of the inspiration for my current setup is lifted from [SeaOrm's Axum Example](https://github.com/SeaQL/sea-orm/tree/master/examples/axum_example). I noticed that they created separate lib crates for each part of the project(entity, service, migration...). This made some sense to me as migration is forced to be a separate crate by SeaOrm, and the setup looks nice. I'm just not sure if it's overkill or what would be me the most conventional way to setup a web server project in Rust.

## Current State of the Website

![A screenshot showing the current state of the website.](<static/assets/april27th_state.png>)