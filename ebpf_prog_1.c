#include <linux/bpf.h>

#ifndef __section
# define __section(x)  __attribute__((section(x), used))
#endif

__section("classifier") int cls_main(struct __sk_buff *skb)
{
        return -1;
}

__section("classifier2") int cls_main2(struct __sk_buff *skb)
{
        return -1;
}

char __license[] __section("license") = "GPL";
