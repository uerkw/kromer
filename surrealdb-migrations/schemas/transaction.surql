DEFINE TABLE OVERWRITE transaction TYPE NORMAL SCHEMAFULL PERMISSIONS NONE;

DEFINE FIELD OVERWRITE amount ON transaction TYPE decimal PERMISSIONS FULL;
DEFINE FIELD OVERWRITE from ON transaction TYPE record<wallet> PERMISSIONS FULL;
DEFINE FIELD OVERWRITE metadata ON transaction TYPE option<string> PERMISSIONS FULL;
DEFINE FIELD OVERWRITE timestamp ON transaction TYPE datetime DEFAULT time::now() PERMISSIONS FULL;
DEFINE FIELD OVERWRITE to ON transaction TYPE record<wallet> | record<name> PERMISSIONS FULL;
DEFINE FIELD OVERWRITE transaction_type ON transaction TYPE 'unknown' | 'mined' | 'name_purchase' | 'transfer' PERMISSIONS FULL;

