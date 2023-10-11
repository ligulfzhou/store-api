ssh hosthatch << EOFHOSTHATCH
ssh ovh << EOFOVH
cd /home/debian/erp-api
git pull origin main
/home/debian/.cargo/bin/cargo build -r
/usr/bin/svc restart erp-9100
EOFOVH
EOFHOSTHATCH
