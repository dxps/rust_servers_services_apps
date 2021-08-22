
drop table if exists ezy_tutor_c6;

create table ezy_tutor_c6 (
    tutor_id serial primary key,
    tutor_name varchar(200) not null,
    tutor_pic_url varchar(200) not null,
    tutor_profile varchar(2000) not null
);
