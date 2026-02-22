// Package netutil provides RFC 1918 private network validation and generation utilities.
package netutil

import (
	"math/rand/v2"
	"net/netip"
)

// rfc1918Prefixes contains the three RFC 1918 private address ranges.
var rfc1918Prefixes = [3]netip.Prefix{
	netip.MustParsePrefix("10.0.0.0/8"),
	netip.MustParsePrefix("172.16.0.0/12"),
	netip.MustParsePrefix("192.168.0.0/16"),
}

// IsRFC1918Addr checks whether addr falls within any of the three RFC 1918 ranges.
func IsRFC1918Addr(addr netip.Addr) bool {
	for _, p := range rfc1918Prefixes {
		if p.Contains(addr) {
			return true
		}
	}
	return false
}

// IsRFC1918Prefix returns true when the entire prefix is within RFC 1918 space.
// Both the network address and the last address of the prefix must be RFC 1918.
func IsRFC1918Prefix(p netip.Prefix) bool {
	masked := p.Masked()
	first := masked.Addr()
	if !IsRFC1918Addr(first) {
		return false
	}
	last := lastAddr(masked)
	return IsRFC1918Addr(last)
}

// lastAddr computes the last (broadcast-equivalent) address in a prefix.
// Returns the zero Addr if the prefix is not IPv4.
func lastAddr(p netip.Prefix) netip.Addr {
	addr := p.Addr()
	if !addr.Is4() {
		return netip.Addr{}
	}
	raw := addr.As4()
	bits := p.Bits()
	// Set all host bits to 1
	for i := bits; i < 32; i++ {
		byteIdx := i / 8
		bitIdx := 7 - (i % 8)
		raw[byteIdx] |= 1 << uint(bitIdx)
	}
	return netip.AddrFrom4(raw)
}

// RandomClassAPrefix generates a random 10.x.y.0/24 prefix.
func RandomClassAPrefix(rng *rand.Rand) netip.Prefix {
	b2 := byte(rng.IntN(254) + 1) // 1-254
	b3 := byte(rng.IntN(254) + 1) // 1-254
	addr := netip.AddrFrom4([4]byte{10, b2, b3, 0})
	return netip.PrefixFrom(addr, 24)
}

// RandomClassBPrefix generates a random 172.[16-31].x.0/24 prefix.
func RandomClassBPrefix(rng *rand.Rand) netip.Prefix {
	b2 := byte(rng.IntN(16) + 16) // 16-31
	b3 := byte(rng.IntN(254) + 1) // 1-254
	addr := netip.AddrFrom4([4]byte{172, b2, b3, 0})
	return netip.PrefixFrom(addr, 24)
}

// RandomClassCPrefix generates a random 192.168.x.0/24 prefix.
func RandomClassCPrefix(rng *rand.Rand) netip.Prefix {
	b3 := byte(rng.IntN(254) + 1) // 1-254
	addr := netip.AddrFrom4([4]byte{192, 168, b3, 0})
	return netip.PrefixFrom(addr, 24)
}
