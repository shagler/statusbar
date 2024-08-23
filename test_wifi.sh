echo "=== Network Interfaces ==="
ip link show

echo -e "\n=== IP Addresses ==="
ip addr show

echo -e "\n=== Routing Table ==="
ip route

echo -e "\n=== Interface Status ==="
for interface in $(ls /sys/class/net); do
    echo -n "$interface: "
    if [ "$(cat /sys/class/net/$interface/operstate)" = "up" ]; then
        echo -n "UP "
        if [ "$(cat /sys/class/net/$interface/carrier)" = "1" ]; then
            echo "CARRIER"
        else
            echo "NO CARRIER"
        fi
    else
        echo "DOWN"
    fi
done

echo -e "\n=== Network Statistics ==="
netstat -i

echo -e "\n=== Active Internet Connections ==="
ss -tuln

echo -e "\n=== Wireless Information ==="
iwconfig 2>/dev/null || echo "No wireless interfaces found"

echo -e "\n=== NetworkManager Status ==="
systemctl is-active NetworkManager
