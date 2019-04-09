#mariadb
CREATE IF NOT EXISTS paintings;

USE paintings;

CREATE TABLE IF NOT EXISTS works (
	id INT UNSIGNED AUTO_INCREMENT, 
	work_name VARCHAR(500),
	creation_date DATE, # primarily year
	location TEXT, # who has it or last known location
	bequeathment VARCHAR(500),
	source VARCHAR(500), #what or where is this a painting of
	medium VARCHAR(100), #what is this work made of
	comment MEDIUMTEXT, #whatever
	current_fk_revision_id INT, # if there is a revision type what is the most current one
	PRIMARY KEY (id) 
);


CREATE TABLE IF NOT EXISTS pics ( 
    pic_id INT UNSIGNED AUTO_INCREMENT, 
    pic_path VARCHAR(500), # path to external file store; paths for easier file movement/backup and performance reasons, could be dangerous to manage
    quality BOOLEAN, # need improvement state?
    fk_works_id INT,
    #fk_revision_id INT 
    PRIMARY KEY (pic_id)
);

CREATE TABLE IF NOT EXISTS locations ( 
    location_id INT UNSIGNED AUTO_INCREMENT, 
    location_date DATE,
    location VARCHAR(500),
    comment MEDIUMTEXT,
    fk_works_id INT, 
    PRIMARY KEY (location_id)
    # should we track completion 
);

/*
CREATE TABLE IF NOT EXISTS revisions ( # revision type for tracking updates 
    revision_id INT UNSIGNED, # will be? work_id plus revision number i.e. work_id = 4  41 42 43, work_id = 5 5_1 5_2 5_3 ... 519
    update_date DATE,
    fk_works_id INT, 
    PRIMARY KEY (revision_id)
    # should we track completion 
);
*/

