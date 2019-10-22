# plano-docente-api

API Projeto Plano Docente


## Database Setup

	export DATABASE_URL="mysql://root:teste@127.0.0.1/PDC"
	export ROCKET_DATABASES="{ PDC = { url = \"$DATABASE_URL\" } }"

## Users routes

### CREATE user
	POST - localhost:8000/users
### READ all users
	GET - localhost:8000/users 
### READ user by id
	GET - localhost:8000/users/<id>
### UPDATE user
	PUT - localhost:8000/users/<id>
### DELETE user
	DELETE - localhost:8000/users/<id>
