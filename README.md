This project is built to provide an extension on the site https://akigeo.com/wieviele/ to allow multiple users to enter names at the same time. 
This is achieved via a separate website for users to enter names in, from where these names will be entered into a database, from which a webscript will regularly pull the data, and insert it into the website.

The project consists of the following parts:
- The Website (Frontend and Backend via Leptos)
- The Backend for the webscript (written with Rocket)
- The webscript (written in JS for greasemonkey (alternatively should also work in tampermonkey))
- The database setup documents (written with SQL)
- A docker-compose setup to allow easy setup via docker-compose with nginx included
- A backend-management project for generating the hash to a targeted password. 

# Important Note
This project is not affiliated in any way with akigeo.com. No code of akigeo.com was used to produce this project, nor do I claim ownership over any parts of akigeo.com. 
Care was taken to reduce the speed of traffic to akigeo.com to prevent server overload (see webscript, rate-limit).

# Description of individual parts
## Website
The Website consist of a frontend and backend written in rust using the laptos framework (a fullstack framework). It provides a rudementary website, without styling.
Data entered into the website will be added into the database, from where it will be pulled by the webscript. 

Names are filtered for duplicates to prevent sending the same name to the webscript multiple times.

## Backend for the webscript
The backend for the webscript is written in rust using the rocket framework. It provides access to parts of the database via specific routes. 
Of Note: All access to the database is handled by the backend via sqlx, so no direct access to the database is possible. This means, that the backend only provides access via the specified api.

The webscript must login via a given password and username, after which it will gain an access token for subsequent authentication. 
The password is compared with the locally stored password hash via argon2. This means that the set password will never be stored unencrypted in the database. 

## Webscript
The webscript is written in raw js as a script for tempermonkey. 
It pulls data regularly from the backend and enters the new names into akigeo.com.

Currently the name is entered a total of REPEAT_AMOUNT times to check for duplicates. 
This is fine in terms of traffic, because akigeo.com will only send new requests to the server, if there is still a new entry for the name. 
This means that the repeats will only cause new traffic, if there is still a new duplicate available.

Already entered names are cached on the session, and requests only pull new data from the backend to reduce traffic. 
Sadly as far as I know there is no way to store data with tempermonkey between reloads of the website, thus the script will request all entries from the database again after a reload.
This may be fixed in the future by storing the state of the webscript on the server.

## Backend User Management
This folder contains a small rust script to generate a hashed string for a given password. This string can then be entered into the database. Currently there is no automated way to insert passwords into the database
so the data has to be entered manually on the final server.

# Deployment
The project is built to be deployed easily using docker.

When deploying make sure to change the domain in the nginx.conf and the webscript. After deployement add a username and password into the database manually and add them to the webscript to allow the webscript to connect.

# Note on passwords
Sample passwords that are used in this project were generated using a random password generator. I would strongly recommend changing them for security purpose before deployement. In future updates it is planned to move all credentials and configuration to environment files or variables for docker-compose, to centralize them.

# Declaration of AI Usage
This project was build without direct usage of generative AI. I have no knowledge on the amount of usage of generative AI in dependencies of this project. 
