# List all network interfaces
echo "=== Network Interfaces ==="
ip link show

# Show IP addresses for all interfaces
echo -e "\n=== IP Addresses ==="
ip addr show

# Display routing table
echo -e "\n=== Routing Table ==="
ip route

# Check which interfaces are up and have carriers
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

# Display network statistics
echo -e "\n=== Network Statistics ==="
netstat -i

# Show active internet connections
echo -e "\n=== Active Internet Connections ==="
ss -tuln

# Display wireless information (if applicable)
echo -e "\n=== Wireless Information ==="
iwconfig 2>/dev/null || echo "No wireless interfaces found"

# Check if NetworkManager is running
echo -e "\n=== NetworkManager Status ==="
systemctl is-active NetworkManager
