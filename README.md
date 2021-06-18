# kellerkompanie-sync-rust
Takes a folder full of addons as input and creates a file index including file hashes, paths and sizes.

Example of an input folder containing addon folders:
```
arma3server@mail:~$ ls -1 /home/arma3server/serverfiles/mods
@3CB_BAF
'@3CB Factions'
@3denEnhanced
@40k
'@80s Tanoa [Global Mobilization Ver.]'
@a3bw
@ace
'@ACE3 - BWMod Compatibility'
@ace_compat_rhs_afrf3

[...]
```

After running kekosync it will create an index.json:
```json
arma3server@mail:~$ head -n 30 kellerkompanie-sync-rust/kellerkompanie-sync/index.json
{
  "addon_groups": [],
  "files_index": {
    "02bcc928-1f74-428c-a8bb-5d9e121b3312": {
      "files": {
        "@HAFM_Navy/HAFM.jpg": {
          "hash": "8AFF5F1A023C875361012F70895DA782E20FEB141AD7AD457D7423C1C33A9341",
          "path": "@HAFM_Navy/HAFM.jpg",
          "size": 245117
        },
        "@HAFM_Navy/HAFM.jpg.zsync": {
          "hash": "3A60F6446A0BE2F3BA59DE33430DB6156937692D68EBE61C3EBB4AB5B1ACF24D",
          "path": "@HAFM_Navy/HAFM.jpg.zsync",
          "size": 458
        },
        "@HAFM_Navy/Naval Weapon Systems.pdf": {
          "hash": "39C6DFFE924CAEE29574392423378033C78AA5A4911F4C1068B6DB9EB6FBC2C5",
          "path": "@HAFM_Navy/Naval Weapon Systems.pdf",
          "size": 1541231
        },
        "@HAFM_Navy/Naval Weapon Systems.pdf.zsync": {
          "hash": "26942DE348216205CCF75F7F3B39397CE66492E3F98BF73D234618AC0CE9D2F5",
          "path": "@HAFM_Navy/Naval Weapon Systems.pdf.zsync",
          "size": 1793
        },
        "@HAFM_Navy/addons/hafm_naval_weapons.pbo": {
          "hash": "E1B50A7285AB94F3AC7ECF861BC1E9C7E1B82183E58C82657B22A4912B14FAE0",
          "path": "@HAFM_Navy/addons/hafm_naval_weapons.pbo",
          "size": 7071540
        },

[...]
```

The index.json in turn is used by the kekosync client to determine which files are up-to-date and which ones need to be downloaded.


## Build prerequisites
1. Rust (including `cargo`), see https://www.rust-lang.org/ for more information.
2. git, see https://git-scm.com/ for more information.


## Build
1. Clone the repository `git clone https://github.com/kellerkompanie/kellerkompanie-sync-rust.git`
2. `cd kellerkompanie-sync-rust/kellerkompanie-sync`
3. `cargo build`


## Running
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
    addon_foldername VARCHAR(128) NOT NULL,
    addon_name VARCHAR(128) NOT NULL,
    CONSTRAINT uuid_unique UNIQUE (addon_uuid),
    CONSTRAINT foldername_unique UNIQUE (addon_foldername)
)
CHARACTER SET 'utf8'
COLLATE 'utf8_german2_ci';


CREATE TABLE IF NOT EXISTS addon_group (
    addon_group_id INT NOT NULL PRIMARY KEY AUTO_INCREMENT,
    addon_group_uuid CHAR(36) NOT NULL,
    addon_group_version CHAR(15) NOT NULL,
    addon_group_name VARCHAR(128) NOT NULL,
    addon_group_author VARCHAR(128) NOT NULL,
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
    addon_category_name VARCHAR(128) NOT NULL,
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
