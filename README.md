# kellerkompanie-sync-rust

## Prerequisites
1. Install Rust (including `cargo`), see https://www.rust-lang.org/ for more information.
2. Clone repository `git clone https://github.com/kellerkompanie/kellerkompanie-sync-rust.git`

## How to build
1. `cd kellerkompanie-sync-rust/kellerkompanie-sync`
2. `cargo build`

## How to run
1. Create a script, e.g., `run-kekosync.sh` with the following contents:
```bash
cd kellerkompanie-sync-rust/kellerkompanie-sync
git pull
cargo run
```
2. Make the script executable using `chmod +x run-kekosync.sh`
3. Run script by invoking `./run-kekosync.sh`

## SQL table definitions
```sql
CREATE TABLE IF NOT EXISTS addon (
    addon_id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    addon_uuid CHAR(36) NOT NULL,
    addon_version CHAR(15) NOT NULL,
    addon_foldername VARCHAR(64) NOT NULL,
    addon_name VARCHAR(64) NOT NULL,
    CONSTRAINT uuid_unique UNIQUE (addon_uuid),
    CONSTRAINT foldername_unique UNIQUE (addon_foldername)
)
CHARACTER SET 'utf8'
COLLATE 'utf8_german2_ci';


CREATE TABLE IF NOT EXISTS addon_group (
    addon_group_id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    addon_group_uuid CHAR(36) NOT NULL,
    addon_group_version CHAR(15) NOT NULL,
    addon_group_name VARCHAR(64) NOT NULL,
    addon_group_author VARCHAR(64) NOT NULL,
    CONSTRAINT uuid_unique UNIQUE (addon_group_uuid),
    CONSTRAINT foldername_unique UNIQUE (addon_group_name)
)
CHARACTER SET 'utf8'
COLLATE 'utf8_german2_ci';


CREATE TABLE IF NOT EXISTS addon_group_member (
    addon_group_id INT NOT NULL,
    addon_id INT NOT NULL,
    CONSTRAINT addon_group_member_pk PRIMARY KEY (addon_group_id,addon_id),
    FOREIGN KEY (addon_group_id) REFERENCES addon_group(addon_group_id),
    FOREIGN KEY (addon_id) REFERENCES addon(addon_id)
);


CREATE TABLE IF NOT EXISTS addon_dependency (
    addon_id INT NOT NULL,
    addon_dependency INT NOT NULL,
    CONSTRAINT addon_dependency_pk PRIMARY KEY (addon_id,addon_dependency),
    FOREIGN KEY (addon_id) REFERENCES addon(addon_id),
    FOREIGN KEY (addon_dependency) REFERENCES addon(addon_id),
    CONSTRAINT dependency_unique UNIQUE (addon_id,addon_dependency)
);

CREATE TABLE IF NOT EXISTS addon_category (
    addon_category_id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    addon_category_name VARCHAR(64) NOT NULL,
    CONSTRAINT addon_category_name_unique UNIQUE (addon_category_name)
)
CHARACTER SET 'utf8'
COLLATE 'utf8_german2_ci';

CREATE TABLE IF NOT EXISTS addon_category_member (
    addon_category_id INT NOT NULL,
    addon_id INT NOT NULL,
    CONSTRAINT addon_category_member_pk PRIMARY KEY (addon_category_id,addon_id),
    FOREIGN KEY (addon_category_id) REFERENCES addon_category(addon_category_id),
    FOREIGN KEY (addon_id) REFERENCES addon(addon_id)
);
```
