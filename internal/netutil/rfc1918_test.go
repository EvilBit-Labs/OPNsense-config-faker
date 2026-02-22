package netutil_test

import (
	"math/rand/v2"
	"net/netip"
	"testing"

	"github.com/EvilBit-Labs/opnsense-config-faker/internal/netutil"
)

func TestIsRFC1918Addr_ValidAddresses(t *testing.T) {
	tests := []struct {
		name string
		addr string
	}{
		{"class A low", "10.0.0.1"},
		{"class A high", "10.255.255.254"},
		{"class B low", "172.16.0.1"},
		{"class B high", "172.31.255.254"},
		{"class C low", "192.168.0.1"},
		{"class C high", "192.168.255.254"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			addr := netip.MustParseAddr(tt.addr)
			if !netutil.IsRFC1918Addr(addr) {
				t.Errorf("expected %s to be RFC 1918", tt.addr)
			}
		})
	}
}

func TestIsRFC1918Addr_PublicAddresses(t *testing.T) {
	tests := []struct {
		name string
		addr string
	}{
		{"google dns", "8.8.8.8"},
		{"cloudflare", "1.1.1.1"},
		{"below class B", "172.15.0.1"},
		{"above class B", "172.32.0.1"},
		{"public 11.x", "11.0.0.1"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			addr := netip.MustParseAddr(tt.addr)
			if netutil.IsRFC1918Addr(addr) {
				t.Errorf("expected %s to NOT be RFC 1918", tt.addr)
			}
		})
	}
}

func TestIsRFC1918Prefix_Valid(t *testing.T) {
	tests := []struct {
		name   string
		prefix string
	}{
		{"class A /24", "10.1.2.0/24"},
		{"class A /8", "10.0.0.0/8"},
		{"class B /24", "172.16.0.0/24"},
		{"class B /12", "172.16.0.0/12"},
		{"class C /24", "192.168.1.0/24"},
		{"class C /16", "192.168.0.0/16"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			p := netip.MustParsePrefix(tt.prefix)
			if !netutil.IsRFC1918Prefix(p) {
				t.Errorf("expected prefix %s to be RFC 1918", tt.prefix)
			}
		})
	}
}

func TestIsRFC1918Prefix_Invalid(t *testing.T) {
	tests := []struct {
		name   string
		prefix string
	}{
		{"public /24", "8.8.8.0/24"},
		{"below class B", "172.15.0.0/24"},
		{"above class B", "172.32.0.0/24"},
	}
	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			p := netip.MustParsePrefix(tt.prefix)
			if netutil.IsRFC1918Prefix(p) {
				t.Errorf("expected prefix %s to NOT be RFC 1918", tt.prefix)
			}
		})
	}
}

func TestRandomClassAPrefix(t *testing.T) {
	rng := rand.New(rand.NewPCG(42, 0))
	for i := range 20 {
		p := netutil.RandomClassAPrefix(rng)
		if !netutil.IsRFC1918Prefix(p) {
			t.Errorf("iteration %d: random class A prefix %s is not RFC 1918", i, p)
		}
		if !p.Addr().Is4() {
			t.Fatalf("iteration %d: expected IPv4 address", i)
		}
		raw := p.Addr().As4()
		if raw[0] != 10 {
			t.Errorf("iteration %d: expected first octet 10, got %d", i, raw[0])
		}
		if p.Bits() != 24 {
			t.Errorf("iteration %d: expected /24, got /%d", i, p.Bits())
		}
	}
}

func TestRandomClassBPrefix(t *testing.T) {
	rng := rand.New(rand.NewPCG(42, 0))
	for i := range 20 {
		p := netutil.RandomClassBPrefix(rng)
		if !netutil.IsRFC1918Prefix(p) {
			t.Errorf("iteration %d: random class B prefix %s is not RFC 1918", i, p)
		}
		if !p.Addr().Is4() {
			t.Fatalf("iteration %d: expected IPv4 address", i)
		}
		raw := p.Addr().As4()
		if raw[0] != 172 {
			t.Errorf("iteration %d: expected first octet 172, got %d", i, raw[0])
		}
		if raw[1] < 16 || raw[1] > 31 {
			t.Errorf("iteration %d: expected second octet 16-31, got %d", i, raw[1])
		}
		if p.Bits() != 24 {
			t.Errorf("iteration %d: expected /24, got /%d", i, p.Bits())
		}
	}
}

func TestRandomClassCPrefix(t *testing.T) {
	rng := rand.New(rand.NewPCG(42, 0))
	for i := range 20 {
		p := netutil.RandomClassCPrefix(rng)
		if !netutil.IsRFC1918Prefix(p) {
			t.Errorf("iteration %d: random class C prefix %s is not RFC 1918", i, p)
		}
		if !p.Addr().Is4() {
			t.Fatalf("iteration %d: expected IPv4 address", i)
		}
		raw := p.Addr().As4()
		if raw[0] != 192 || raw[1] != 168 {
			t.Errorf("iteration %d: expected 192.168.x.0, got %v", i, raw)
		}
		if p.Bits() != 24 {
			t.Errorf("iteration %d: expected /24, got /%d", i, p.Bits())
		}
	}
}
