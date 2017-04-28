# cd to project
cd /home/markm/Documents/journey

git pull

make clean
make -j

############################################
############################################

# set processor frequency
./scaling_set_freq.sh 2400000

# print pid of kswapd
pgrep kswapd
pgrep khugepaged

# print hugepage settings
cat /sys/kernel/mm/transparent_hugepage/enabled

./bench_cpu_kswapd_anon 18 $(pgrep kswapd) > results/cpu/cpu_kswapd_cont_vm_18_swap_32_hugepages_always

reboot
##sleep
##reconnect

############################################
############################################

# set processor frequency
./scaling_set_freq.sh 2400000

# print pid of kswapd
pgrep kswapd
pgrep khugepaged

# print hugepage settings
cat /sys/kernel/mm/transparent_hugepage/enabled

./bench_cpu_kswapd_anon_rand 18 $(pgrep kswapd) > results/cpu/cpu_kswapd_rand_vm_18_swap_32_hugepages_always

reboot
##sleep
##reconnect

############################################
############################################

# set processor frequency
./scaling_set_freq.sh 2400000

# print pid of kswapd
pgrep kswapd
pgrep khugepaged

# print hugepage settings
cat /sys/kernel/mm/transparent_hugepage/enabled

./bench_cpu_kswapd_anon 18 $(pgrep khugepaged) > results/cpu/cpu_khugepaged_cont_vm_18_swap_32_hugepages_always

reboot
##sleep
##reconnect

############################################
############################################

# set processor frequency
./scaling_set_freq.sh 2400000

# print pid of kswapd
pgrep kswapd
pgrep khugepaged

# print hugepage settings
cat /sys/kernel/mm/transparent_hugepage/enabled

./bench_cpu_kswapd_anon_rand 18 $(pgrep khugepaged) > results/cpu/cpu_khugepaged_rand_vm_18_swap_32_hugepages_always

reboot
##sleep
##reconnect

#./bench_cpu_kswapd_anon 18 $(pgrep kswapd) > results/cpu/cpu_kswapd_cont_vm_18_swap_32_hugepages_never
#./bench_cpu_kswapd_anon_rand 18 $(pgrep kswapd) > results/cpu/cpu_kswapd_rand_vm_18_swap_32_hugepages_never
