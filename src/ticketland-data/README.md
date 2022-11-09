Running Docker
===

Start Colima

or `colima start --cpu 2 --memory 8 --disk 50 --dns 8.8.8.8 --network-address`

We can get the IP address bby running

`colima ls`

```
PROFILE    STATUS     ARCH       CPUS    MEMORY    DISK     RUNTIME       ADDRESS
default    Running    aarch64    4       8GiB      50GiB    docker+k3s    192.xxx.xxx.x
```

```bash
docker volume create pgdata

docker run -d \
	--name pg \
  -p 5432:5432 \
	-e POSTGRES_PASSWORD=password \
	-v pgdata:/var/lib/postgresql/data \
	postgres
```

Diesel
===

Setup 

`diesel setup --database-url postgres://postgres:password@localhost/ticketland`
