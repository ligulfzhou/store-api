ssh ovh << EOFOVH
cd /home/debian/store-api
git pull origin main
/home/debian/.cargo/bin/cargo build -r
/usr/bin/svc restart lkx:lkx-9200
/usr/bin/svc restart lkx:lkx-9201
/usr/bin/svc restart lkx:lkx-9202
/usr/bin/svc restart lkx:lkx-9203
EOFOVH
