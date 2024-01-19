ssh ovh << EOFOVH
cd /home/debian/store-api
git pull origin main
/home/debian/.cargo/bin/cargo build -r
/usr/bin/svc restart lkx:lkx-9200
EOFOVH
