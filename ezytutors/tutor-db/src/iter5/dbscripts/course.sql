drop table if exists ezy_course_c6;

create table ezy_course_c6
(
   course_id            serial primary key,
   tutor_id             INT not null,
   course_name          varchar(140) not null,
   course_description   varchar(2000),
   course_format        varchar(30),
   course_structure     varchar(200),
   course_duration      varchar(30),
   course_price         INT,
   course_language      varchar(30),
   course_level         varchar(30),
   posted_time          TIMESTAMP default now(),
   CONSTRAINT fk_tutor  FOREIGN KEY(tutor_id) REFERENCES ezy_tutor_c6(tutor_id) 
);
