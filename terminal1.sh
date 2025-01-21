
API_SOCKET="/tmp/firecracker.socket"

# Remove API unix socket
sudo rm -f $API_SOCKET

# Run firecracker
sudo ./firecracker --api-sock "${API_SOCKET}"
#sudo strace -ff -o fc-strace.log ./firecracker --api-sock "${API_SOCKET}"
