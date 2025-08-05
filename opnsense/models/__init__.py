from opnsense.models.acls import Acls
from opnsense.models.action import Action
from opnsense.models.active_interface import ActiveInterface
from opnsense.models.active_timeout import ActiveTimeout
from opnsense.models.address import Address
from opnsense.models.adv_dhcp6_authentication_statement_algorithm import (
    AdvDhcp6AuthenticationStatementAlgorithm,
)
from opnsense.models.adv_dhcp6_authentication_statement_authname import (
    AdvDhcp6AuthenticationStatementAuthname,
)
from opnsense.models.adv_dhcp6_authentication_statement_protocol import (
    AdvDhcp6AuthenticationStatementProtocol,
)
from opnsense.models.adv_dhcp6_authentication_statement_rdm import (
    AdvDhcp6AuthenticationStatementRdm,
)
from opnsense.models.adv_dhcp6_config_advanced import AdvDhcp6ConfigAdvanced
from opnsense.models.adv_dhcp6_config_file_override import (
    AdvDhcp6ConfigFileOverride,
)
from opnsense.models.adv_dhcp6_config_file_override_path import (
    AdvDhcp6ConfigFileOverridePath,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_address import (
    AdvDhcp6IdAssocStatementAddress,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_address_enable import (
    AdvDhcp6IdAssocStatementAddressEnable,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_address_id import (
    AdvDhcp6IdAssocStatementAddressId,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_address_pltime import (
    AdvDhcp6IdAssocStatementAddressPltime,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_address_vltime import (
    AdvDhcp6IdAssocStatementAddressVltime,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_prefix import (
    AdvDhcp6IdAssocStatementPrefix,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_prefix_enable import (
    AdvDhcp6IdAssocStatementPrefixEnable,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_prefix_id import (
    AdvDhcp6IdAssocStatementPrefixId,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_prefix_pltime import (
    AdvDhcp6IdAssocStatementPrefixPltime,
)
from opnsense.models.adv_dhcp6_id_assoc_statement_prefix_vltime import (
    AdvDhcp6IdAssocStatementPrefixVltime,
)
from opnsense.models.adv_dhcp6_interface_statement_information_only_enable import (
    AdvDhcp6InterfaceStatementInformationOnlyEnable,
)
from opnsense.models.adv_dhcp6_interface_statement_request_options import (
    AdvDhcp6InterfaceStatementRequestOptions,
)
from opnsense.models.adv_dhcp6_interface_statement_script import (
    AdvDhcp6InterfaceStatementScript,
)
from opnsense.models.adv_dhcp6_interface_statement_send_options import (
    AdvDhcp6InterfaceStatementSendOptions,
)
from opnsense.models.adv_dhcp6_key_info_statement_expire import (
    AdvDhcp6KeyInfoStatementExpire,
)
from opnsense.models.adv_dhcp6_key_info_statement_keyid import (
    AdvDhcp6KeyInfoStatementKeyid,
)
from opnsense.models.adv_dhcp6_key_info_statement_keyname import (
    AdvDhcp6KeyInfoStatementKeyname,
)
from opnsense.models.adv_dhcp6_key_info_statement_realm import (
    AdvDhcp6KeyInfoStatementRealm,
)
from opnsense.models.adv_dhcp6_key_info_statement_secret import (
    AdvDhcp6KeyInfoStatementSecret,
)
from opnsense.models.adv_dhcp6_prefix_interface_statement_sla_len import (
    AdvDhcp6PrefixInterfaceStatementSlaLen,
)
from opnsense.models.adv_dhcp_config_advanced import AdvDhcpConfigAdvanced
from opnsense.models.adv_dhcp_config_file_override import (
    AdvDhcpConfigFileOverride,
)
from opnsense.models.adv_dhcp_config_file_override_path import (
    AdvDhcpConfigFileOverridePath,
)
from opnsense.models.adv_dhcp_option_modifiers import AdvDhcpOptionModifiers
from opnsense.models.adv_dhcp_pt_backoff_cutoff import AdvDhcpPtBackoffCutoff
from opnsense.models.adv_dhcp_pt_initial_interval import (
    AdvDhcpPtInitialInterval,
)
from opnsense.models.adv_dhcp_pt_reboot import AdvDhcpPtReboot
from opnsense.models.adv_dhcp_pt_retry import AdvDhcpPtRetry
from opnsense.models.adv_dhcp_pt_select_timeout import AdvDhcpPtSelectTimeout
from opnsense.models.adv_dhcp_pt_timeout import AdvDhcpPtTimeout
from opnsense.models.adv_dhcp_pt_values import AdvDhcpPtValues
from opnsense.models.adv_dhcp_request_options import AdvDhcpRequestOptions
from opnsense.models.adv_dhcp_required_options import AdvDhcpRequiredOptions
from opnsense.models.adv_dhcp_send_options import AdvDhcpSendOptions
from opnsense.models.advanced import Advanced
from opnsense.models.aggressivensec import Aggressivensec
from opnsense.models.alert import Alert
from opnsense.models.alert_logrotate import AlertLogrotate
from opnsense.models.alert_save_logs import AlertSaveLogs
from opnsense.models.alias import Alias
from opnsense.models.alias_address import AliasAddress
from opnsense.models.alias_subnet import AliasSubnet
from opnsense.models.aliases import Aliases
from opnsense.models.any import AnyType
from opnsense.models.apikeys import Apikeys
from opnsense.models.app import App
from opnsense.models.asn import Asn
from opnsense.models.authorizedkeys import Authorizedkeys
from opnsense.models.blockbogons import Blockbogons
from opnsense.models.blocklists import Blocklists
from opnsense.models.blockpriv import Blockpriv
from opnsense.models.bogons import Bogons
from opnsense.models.bridged import Bridged
from opnsense.models.bridges import Bridges
from opnsense.models.ca import Ca
from opnsense.models.cacheflush import Cacheflush
from opnsense.models.cachemaxnegativettl import Cachemaxnegativettl
from opnsense.models.cachemaxttl import Cachemaxttl
from opnsense.models.cacheminttl import Cacheminttl
from opnsense.models.captiveportal import Captiveportal
from opnsense.models.capture import Capture
from opnsense.models.categories import Categories
from opnsense.models.category_1 import Category1
from opnsense.models.category_2 import Category2
from opnsense.models.cert import Cert
from opnsense.models.cfg import Cfg
from opnsense.models.charon import Charon
from opnsense.models.chd import Chd
from opnsense.models.children import Children
from opnsense.models.cipher_string import CipherString
from opnsense.models.ciphersuites import Ciphersuites
from opnsense.models.client import (
    Client,
    Clients,
)
from opnsense.models.clone import Clone
from opnsense.models.code import Code
from opnsense.models.collect import Collect
from opnsense.models.column_count import ColumnCount
from opnsense.models.condition import Condition
from opnsense.models.connections import Connections
from opnsense.models.created import Created
from opnsense.models.cron import Cron
from opnsense.models.crt import Crt
from opnsense.models.ctrl_agent import CtrlAgent
from opnsense.models.custom import Custom
from opnsense.models.daemon import Daemon
from opnsense.models.ddnsdomainalgorithm import Ddnsdomainalgorithm
from opnsense.models.default_action import DefaultAction
from opnsense.models.default_packet_size import DefaultPacketSize
from opnsense.models.defaultgw import Defaultgw
from opnsense.models.depends import Depends
from opnsense.models.descr import Descr
from opnsense.models.description import Description
from opnsense.models.destination import Destination
from opnsense.models.destinations import Destinations
from opnsense.models.detect import Detect
from opnsense.models.dhcp4 import Dhcp4
from opnsense.models.dhcp6_ia_pd_len import Dhcp6IaPdLen
from opnsense.models.dhcpd import Dhcpd
from opnsense.models.dhcpdv6 import Dhcpdv6
from opnsense.models.dhcphostname import Dhcphostname
from opnsense.models.dhcprejectfrom import Dhcprejectfrom
from opnsense.models.dhcrelay import Dhcrelay
from opnsense.models.direction import Direction
from opnsense.models.disablechecksumoffloading import Disablechecksumoffloading
from opnsense.models.disableconsolemenu import Disableconsolemenu
from opnsense.models.disabled import Disabled
from opnsense.models.disablelargereceiveoffloading import (
    Disablelargereceiveoffloading,
)
from opnsense.models.disablenatreflection import Disablenatreflection
from opnsense.models.disablepreempt import Disablepreempt
from opnsense.models.disableroutes import Disableroutes
from opnsense.models.disablesegmentationoffloading import (
    Disablesegmentationoffloading,
)
from opnsense.models.disablevlanhwfilter import Disablevlanhwfilter
from opnsense.models.disablevpnrules import Disablevpnrules
from opnsense.models.discardtimeout import Discardtimeout
from opnsense.models.disconnectppps import Disconnectppps
from opnsense.models.dmn import Dmn
from opnsense.models.dns import Dns
from opnsense.models.dns64 import Dns64
from opnsense.models.dns64prefix import Dns64Prefix
from opnsense.models.dnsallowoverride import Dnsallowoverride
from opnsense.models.dnsbl import Dnsbl
from opnsense.models.dnssec import Dnssec
from opnsense.models.dnssecstripped import Dnssecstripped
from opnsense.models.dnsserver import Dnsserver
from opnsense.models.domain import Domain
from opnsense.models.domains import Domains
from opnsense.models.dots import Dots
from opnsense.models.dump_all_headers import DumpAllHeaders
from opnsense.models.egress_only import EgressOnly
from opnsense.models.enable import Enable
from opnsense.models.enable_config_constraints import EnableConfigConstraints
from opnsense.models.enable_legacy_sect import EnableLegacySect
from opnsense.models.enable_wpad import EnableWpad
from opnsense.models.enabled import Enabled
from opnsense.models.enc import Enc
from opnsense.models.esp import Esp
from opnsense.models.eve_log import EveLog
from opnsense.models.eventqueue_path import EventqueuePath
from opnsense.models.eventqueue_slots import EventqueueSlots
from opnsense.models.events import Events
from opnsense.models.expect import Expect
from opnsense.models.expires import Expires
from opnsense.models.extended import Extended
from opnsense.models.extendedstatistics import Extendedstatistics
from opnsense.models.failover_peerip import FailoverPeerip
from opnsense.models.fargw import Fargw
from opnsense.models.fetch_crls import FetchCrls
from opnsense.models.file_tags import FileTags
from opnsense.models.files import Files
from opnsense.models.filter_1 import Filter1
from opnsense.models.filter_2 import Filter2
from opnsense.models.firewall import Firewall
from opnsense.models.firmware import Firmware
from opnsense.models.flavour import Flavour
from opnsense.models.format import Format
from opnsense.models.forwarding import Forwarding
from opnsense.models.from_mod import From
from opnsense.models.fwrules import Fwrules
from opnsense.models.gateway import Gateway
from opnsense.models.gateway_item import GatewayItem
from opnsense.models.gateways_1 import Gateways1
from opnsense.models.gateways_2 import Gateways2
from opnsense.models.gatewayv6 import Gatewayv6
from opnsense.models.general import General
from opnsense.models.geoip import Geoip
from opnsense.models.gid import Gid
from opnsense.models.gif import Gif
from opnsense.models.gifs import Gifs
from opnsense.models.gre import Gre
from opnsense.models.gres import Gres
from opnsense.models.group import Group
from opnsense.models.groupname import Groupname
from opnsense.models.groups import Groups
from opnsense.models.ha import Ha
from opnsense.models.ha_peers import HaPeers
from opnsense.models.hasync import Hasync
from opnsense.models.hideidentity import Hideidentity
from opnsense.models.hideversion import Hideversion
from opnsense.models.homenet import Homenet
from opnsense.models.host import Host
from opnsense.models.hostname import Hostname
from opnsense.models.hosts import Hosts
from opnsense.models.http import Http
from opnsense.models.http_host import HttpHost
from opnsense.models.http_port import HttpPort
from opnsense.models.httpd_allow import HttpdAllow
from opnsense.models.httpd_enabled import HttpdEnabled
from opnsense.models.httpd_password import HttpdPassword
from opnsense.models.httpd_port import HttpdPort
from opnsense.models.httpd_username import HttpdUsername
from opnsense.models.ids import Ids
from opnsense.models.if_mod import If
from opnsense.models.ifgroups import Ifgroups
from opnsense.models.ignore_acquire_ts import IgnoreAcquireTs
from opnsense.models.ike import Ike
from opnsense.models.ike_name import IkeName
from opnsense.models.ikesa_table_segments import IkesaTableSegments
from opnsense.models.ikesa_table_size import IkesaTableSize
from opnsense.models.imc import Imc
from opnsense.models.imv import Imv
from opnsense.models.inactive_timeout import InactiveTimeout
from opnsense.models.incomingnumtcp import Incomingnumtcp
from opnsense.models.infracachenumhosts import Infracachenumhosts
from opnsense.models.infrahostttl import Infrahostttl
from opnsense.models.infrakeepprobing import Infrakeepprobing
from opnsense.models.init_limit_half_open import InitLimitHalfOpen
from opnsense.models.insecuredomain import Insecuredomain
from opnsense.models.install_crls import InstallCrls
from opnsense.models.instance import Instance
from opnsense.models.instances import Instances
from opnsense.models.interface import Interface
from opnsense.models.interfaces_1 import Interfaces1
from opnsense.models.interfaces_2 import Interfaces2
from opnsense.models.internal_dynamic import InternalDynamic
from opnsense.models.interval import Interval
from opnsense.models.ipaddr import Ipaddr
from opnsense.models.ipaddrv6 import Ipaddrv6
from opnsense.models.ipprotocol import Ipprotocol
from opnsense.models.ips import Ips
from opnsense.models.ipsec import Ipsec
from opnsense.models.ipsecpsk import Ipsecpsk
from opnsense.models.ipv6allow import Ipv6Allow
from opnsense.models.item import Item
from opnsense.models.job import Job
from opnsense.models.jobs import Jobs
from opnsense.models.jostletimeout import Jostletimeout
from opnsense.models.kea import Kea
from opnsense.models.keepalive import Keepalive
from opnsense.models.key import Key
from opnsense.models.key_pairs import KeyPairs
from opnsense.models.knl import Knl
from opnsense.models.lagg import Lagg
from opnsense.models.laggs import Laggs
from opnsense.models.lan import Lan
from opnsense.models.language import Language
from opnsense.models.lb_use_sticky import LbUseSticky
from opnsense.models.lib import Lib
from opnsense.models.lists import Lists
from opnsense.models.lo0 import Lo0
from opnsense.models.load_balancer import LoadBalancer
from opnsense.models.local_zone_type import LocalZoneType
from opnsense.models.locals import Locals
from opnsense.models.lock import Lock
from opnsense.models.log_level import LogLevel
from opnsense.models.log_payload import LogPayload
from opnsense.models.logfile import Logfile
from opnsense.models.loglocal import Loglocal
from opnsense.models.loglocalactions import Loglocalactions
from opnsense.models.logqueries import Logqueries
from opnsense.models.logreplies import Logreplies
from opnsense.models.logservfail import Logservfail
from opnsense.models.logtagqueryreply import Logtagqueryreply
from opnsense.models.logverbosity import Logverbosity
from opnsense.models.loopbacks import Loopbacks
from opnsense.models.lvtemplate import Lvtemplate
from opnsense.models.mac import Mac
from opnsense.models.mailserver import Mailserver
from opnsense.models.make_before_break import MakeBeforeBreak
from opnsense.models.match import Match
from opnsense.models.max_ikev1_exchanges import MaxIkev1Exchanges
from opnsense.models.max_unacked_clients import MaxUnackedClients
from opnsense.models.maxfilesize import Maxfilesize
from opnsense.models.maxpreserve import Maxpreserve
from opnsense.models.media import Media
from opnsense.models.mediaopt import Mediaopt
from opnsense.models.member import Member
from opnsense.models.mgr import Mgr
from opnsense.models.min_protocol import MinProtocol
from opnsense.models.min_protocol_dtls import MinProtocolDtls
from opnsense.models.mirror import Mirror
from opnsense.models.mmonit_register_credentials import (
    MmonitRegisterCredentials,
)
from opnsense.models.mmonit_timeout import MmonitTimeout
from opnsense.models.mmonit_url import MmonitUrl
from opnsense.models.mode import Mode
from opnsense.models.monit import Monit
from opnsense.models.monitor_disable import MonitorDisable
from opnsense.models.monitor_type import MonitorType
from opnsense.models.mpmalgo import Mpmalgo
from opnsense.models.msgcachesize import Msgcachesize
from opnsense.models.mtu import Mtu
from opnsense.models.name import Name
from opnsense.models.nat import Nat
from opnsense.models.neighbors import Neighbors
from opnsense.models.net import Net
from opnsense.models.netflow import Netflow
from opnsense.models.netflowbackup import Netflowbackup
from opnsense.models.network import Network
from opnsense.models.networks import Networks
from opnsense.models.nextgid import Nextgid
from opnsense.models.nextuid import Nextuid
from opnsense.models.noarecords import Noarecords
from opnsense.models.noreglladdr6 import Noreglladdr6
from opnsense.models.noregrecords import Noregrecords
from opnsense.models.noton import Noton
from opnsense.models.npt import Npt
from opnsense.models.ntpd import Ntpd
from opnsense.models.ntpserver import Ntpserver
from opnsense.models.number import Number
from opnsense.models.numberoptions import Numberoptions
from opnsense.models.numqueriesperthread import Numqueriesperthread
from opnsense.models.nxdomain import Nxdomain
from opnsense.models.onetoone import Onetoone
from opnsense.models.open_vpn_1 import OpenVpn1
from opnsense.models.open_vpnexport import OpenVpnexport
from opnsense.models.openvpn_2 import Openvpn2
from opnsense.models.opnsense_1 import Opnsense1
from opnsense.models.opnsense_2 import Opnsense2
from opnsense.models.optimization import Optimization
from opnsense.models.options import Options
from opnsense.models.otp_seed import OtpSeed
from opnsense.models.outbound import Outbound
from opnsense.models.outgoing_interface import OutgoingInterface
from opnsense.models.outgoingnumtcp import Outgoingnumtcp
from opnsense.models.outgoingrange import Outgoingrange
from opnsense.models.overwrites import Overwrites
from opnsense.models.passthrough_networks import PassthroughNetworks
from opnsense.models.password import Password
from opnsense.models.path import Path
from opnsense.models.peers import Peers
from opnsense.models.pf_share_forward import PfShareForward
from opnsense.models.pfsyncinterface import Pfsyncinterface
from opnsense.models.pfsyncpeerip import Pfsyncpeerip
from opnsense.models.pfsyncversion import Pfsyncversion
from opnsense.models.pidfile import Pidfile
from opnsense.models.pipes import Pipes
from opnsense.models.plugins import Plugins
from opnsense.models.policies import Policies
from opnsense.models.polltime import Polltime
from opnsense.models.poolopts import Poolopts
from opnsense.models.poolopts_sourcehashkey import PooloptsSourcehashkey
from opnsense.models.pools import Pools
from opnsense.models.port import Port
from opnsense.models.powerd_ac_mode import PowerdAcMode
from opnsense.models.powerd_battery_mode import PowerdBatteryMode
from opnsense.models.powerd_normal_mode import PowerdNormalMode
from opnsense.models.ppp import Ppp
from opnsense.models.ppps import Ppps
from opnsense.models.pre_shared_keys import PreSharedKeys
from opnsense.models.prefer import Prefer
from opnsense.models.preferred_oldsa import PreferredOldsa
from opnsense.models.prefetch import Prefetch
from opnsense.models.prefetchkey import Prefetchkey
from opnsense.models.priv import Priv
from opnsense.models.privateaddress import Privateaddress
from opnsense.models.privatedomain import Privatedomain
from opnsense.models.privkey import Privkey
from opnsense.models.profile import Profile
from opnsense.models.promisc import Promisc
from opnsense.models.protocol import Protocol
from opnsense.models.prv import Prv
from opnsense.models.psk import Psk
from opnsense.models.pts import Pts
from opnsense.models.pubkey import Pubkey
from opnsense.models.qnameminstrict import Qnameminstrict
from opnsense.models.queues import Queues
from opnsense.models.quick import Quick
from opnsense.models.range import Range
from opnsense.models.reboot import Reboot
from opnsense.models.recipient import Recipient
from opnsense.models.refid import Refid
from opnsense.models.regdhcp import Regdhcp
from opnsense.models.regdhcpdomain import Regdhcpdomain
from opnsense.models.regdhcpstatic import Regdhcpstatic
from opnsense.models.reminder import Reminder
from opnsense.models.remotes import Remotes
from opnsense.models.reservations import Reservations
from opnsense.models.retransmit_base import RetransmitBase
from opnsense.models.retransmit_jitter import RetransmitJitter
from opnsense.models.retransmit_limit import RetransmitLimit
from opnsense.models.retransmit_timeout import RetransmitTimeout
from opnsense.models.retransmit_tries import RetransmitTries
from opnsense.models.revision import Revision
from opnsense.models.rocommunity import Rocommunity
from opnsense.models.route import Route
from opnsense.models.rrd import Rrd
from opnsense.models.rrdbackup import Rrdbackup
from opnsense.models.rrsetcachesize import Rrsetcachesize
from opnsense.models.rule import Rule
from opnsense.models.rules import Rules
from opnsense.models.safesearch import Safesearch
from opnsense.models.scope import Scope
from opnsense.models.secret import Secret
from opnsense.models.send import Send
from opnsense.models.sequence import Sequence
from opnsense.models.serveexpired import Serveexpired
from opnsense.models.serveexpiredclienttimeout import Serveexpiredclienttimeout
from opnsense.models.serveexpiredreplyttl import Serveexpiredreplyttl
from opnsense.models.serveexpiredttl import Serveexpiredttl
from opnsense.models.serveexpiredttlreset import Serveexpiredttlreset
from opnsense.models.serveraddress import Serveraddress
from opnsense.models.serverport import Serverport
from opnsense.models.servers import (
    Server,
    Servers,
)
from opnsense.models.service import Service
from opnsense.models.session_resumption import SessionResumption
from opnsense.models.snatrules import Snatrules
from opnsense.models.snmpd import Snmpd
from opnsense.models.source import Source
from opnsense.models.sourceport import Sourceport
from opnsense.models.spds import Spds
from opnsense.models.spoofmac import Spoofmac
from opnsense.models.ssh import Ssh
from opnsense.models.ssl import Ssl
from opnsense.models.ssl_certref import SslCertref
from opnsense.models.sslverify import Sslverify
from opnsense.models.sslversion import Sslversion
from opnsense.models.start import Start
from opnsense.models.startdelay import Startdelay
from opnsense.models.starttimeout import Starttimeout
from opnsense.models.statefile import Statefile
from opnsense.models.statetype import Statetype
from opnsense.models.static_keys import StaticKeys
from opnsense.models.staticmap import Staticmap
from opnsense.models.staticroutes import Staticroutes
from opnsense.models.stats import Stats
from opnsense.models.stop import Stop
from opnsense.models.store_intermediate_certs import StoreIntermediateCerts
from opnsense.models.subnet import Subnet
from opnsense.models.subnets import Subnets
from opnsense.models.subnetv6 import Subnetv6
from opnsense.models.subscription import Subscription
from opnsense.models.swanctl import Swanctl
from opnsense.models.synchronizetoip import Synchronizetoip
from opnsense.models.syncitems import Syncitems
from opnsense.models.syscontact import Syscontact
from opnsense.models.sysctl import Sysctl
from opnsense.models.syslocation import Syslocation
from opnsense.models.syslog_1 import Syslog1
from opnsense.models.syslog_2 import Syslog2
from opnsense.models.syslog_eve import SyslogEve
from opnsense.models.system import System
from opnsense.models.tag import Tag
from opnsense.models.tagged import Tagged
from opnsense.models.target import Target
from opnsense.models.targets import Targets
from opnsense.models.templates import Templates
from opnsense.models.test import Test
from opnsense.models.tests import Tests
from opnsense.models.theme import Theme
from opnsense.models.this_server_name import ThisServerName
from opnsense.models.threads import Threads
from opnsense.models.time import Time
from opnsense.models.timeout import Timeout
from opnsense.models.timeservers import Timeservers
from opnsense.models.timezone import Timezone
from opnsense.models.tls import Tls
from opnsense.models.tnc import Tnc
from opnsense.models.to import To
from opnsense.models.toclient_groups import ToclientGroups
from opnsense.models.toserver_groups import ToserverGroups
from opnsense.models.track6_interface import Track6Interface
from opnsense.models.track6_prefix_id import Track6PrefixId
from opnsense.models.traffic_shaper import TrafficShaper
from opnsense.models.trigger_initial_wizard import TriggerInitialWizard
from opnsense.models.trust import Trust
from opnsense.models.tunable import Tunable
from opnsense.models.tunneladdress import Tunneladdress
from opnsense.models.txtsupport import Txtsupport
from opnsense.models.type_mod import Type
from opnsense.models.uid import Uid
from opnsense.models.unbound import Unbound
from opnsense.models.unboundplus import Unboundplus
from opnsense.models.unwantedreplythreshold import Unwantedreplythreshold
from opnsense.models.update_cron import UpdateCron
from opnsense.models.updated import Updated
from opnsense.models.url import Url
from opnsense.models.user import User
from opnsense.models.user_defined_rules import UserDefinedRules
from opnsense.models.username import Username
from opnsense.models.usevirtualterminal import Usevirtualterminal
from opnsense.models.valid_lifetime import ValidLifetime
from opnsense.models.valloglevel import Valloglevel
from opnsense.models.value import Value
from opnsense.models.verbosity import Verbosity
from opnsense.models.version import Version
from opnsense.models.vip import Vip
from opnsense.models.virtual import Virtual
from opnsense.models.virtualip import Virtualip
from opnsense.models.vlan import Vlan
from opnsense.models.vlans import Vlans
from opnsense.models.vtis import Vtis
from opnsense.models.vxlans import Vxlans
from opnsense.models.wan import Wan
from opnsense.models.webgui import Webgui
from opnsense.models.weight import Weight
from opnsense.models.whitelists import Whitelists
from opnsense.models.widgets import Widgets
from opnsense.models.wildcards import Wildcards
from opnsense.models.winsserver import Winsserver
from opnsense.models.wireguard import Wireguard
from opnsense.models.wireless import Wireless
from opnsense.models.zones import Zones

__all__ = [
    "Acls",
    "Action",
    "ActiveInterface",
    "ActiveTimeout",
    "Address",
    "AdvDhcp6AuthenticationStatementAlgorithm",
    "AdvDhcp6AuthenticationStatementAuthname",
    "AdvDhcp6AuthenticationStatementProtocol",
    "AdvDhcp6AuthenticationStatementRdm",
    "AdvDhcp6ConfigAdvanced",
    "AdvDhcp6ConfigFileOverride",
    "AdvDhcp6ConfigFileOverridePath",
    "AdvDhcp6IdAssocStatementAddress",
    "AdvDhcp6IdAssocStatementAddressEnable",
    "AdvDhcp6IdAssocStatementAddressId",
    "AdvDhcp6IdAssocStatementAddressPltime",
    "AdvDhcp6IdAssocStatementAddressVltime",
    "AdvDhcp6IdAssocStatementPrefix",
    "AdvDhcp6IdAssocStatementPrefixEnable",
    "AdvDhcp6IdAssocStatementPrefixId",
    "AdvDhcp6IdAssocStatementPrefixPltime",
    "AdvDhcp6IdAssocStatementPrefixVltime",
    "AdvDhcp6InterfaceStatementInformationOnlyEnable",
    "AdvDhcp6InterfaceStatementRequestOptions",
    "AdvDhcp6InterfaceStatementScript",
    "AdvDhcp6InterfaceStatementSendOptions",
    "AdvDhcp6KeyInfoStatementExpire",
    "AdvDhcp6KeyInfoStatementKeyid",
    "AdvDhcp6KeyInfoStatementKeyname",
    "AdvDhcp6KeyInfoStatementRealm",
    "AdvDhcp6KeyInfoStatementSecret",
    "AdvDhcp6PrefixInterfaceStatementSlaLen",
    "AdvDhcpConfigAdvanced",
    "AdvDhcpConfigFileOverride",
    "AdvDhcpConfigFileOverridePath",
    "AdvDhcpOptionModifiers",
    "AdvDhcpPtBackoffCutoff",
    "AdvDhcpPtInitialInterval",
    "AdvDhcpPtReboot",
    "AdvDhcpPtRetry",
    "AdvDhcpPtSelectTimeout",
    "AdvDhcpPtTimeout",
    "AdvDhcpPtValues",
    "AdvDhcpRequestOptions",
    "AdvDhcpRequiredOptions",
    "AdvDhcpSendOptions",
    "Advanced",
    "Aggressivensec",
    "Alert",
    "AlertLogrotate",
    "AlertSaveLogs",
    "Alias",
    "AliasAddress",
    "AliasSubnet",
    "Aliases",
    "AnyType",
    "Apikeys",
    "App",
    "Asn",
    "Authorizedkeys",
    "Blockbogons",
    "Blocklists",
    "Blockpriv",
    "Bogons",
    "Bridged",
    "Bridges",
    "Ca",
    "Cacheflush",
    "Cachemaxnegativettl",
    "Cachemaxttl",
    "Cacheminttl",
    "Captiveportal",
    "Capture",
    "Categories",
    "Category1",
    "Category2",
    "Cert",
    "Cfg",
    "Charon",
    "Chd",
    "Children",
    "CipherString",
    "Ciphersuites",
    "Client",
    "Clients",
    "Clone",
    "Code",
    "Collect",
    "ColumnCount",
    "Condition",
    "Connections",
    "Created",
    "Cron",
    "Crt",
    "CtrlAgent",
    "Custom",
    "Daemon",
    "Ddnsdomainalgorithm",
    "DefaultAction",
    "DefaultPacketSize",
    "Defaultgw",
    "Depends",
    "Descr",
    "Description",
    "Destination",
    "Destinations",
    "Detect",
    "Dhcp4",
    "Dhcp6IaPdLen",
    "Dhcpd",
    "Dhcpdv6",
    "Dhcphostname",
    "Dhcprejectfrom",
    "Dhcrelay",
    "Direction",
    "Disablechecksumoffloading",
    "Disableconsolemenu",
    "Disabled",
    "Disablelargereceiveoffloading",
    "Disablenatreflection",
    "Disablepreempt",
    "Disableroutes",
    "Disablesegmentationoffloading",
    "Disablevlanhwfilter",
    "Disablevpnrules",
    "Discardtimeout",
    "Disconnectppps",
    "Dmn",
    "Dns",
    "Dns64",
    "Dns64Prefix",
    "Dnsallowoverride",
    "Dnsbl",
    "Dnssec",
    "Dnssecstripped",
    "Dnsserver",
    "Domain",
    "Domains",
    "Dots",
    "DumpAllHeaders",
    "EgressOnly",
    "Enable",
    "EnableConfigConstraints",
    "EnableLegacySect",
    "EnableWpad",
    "Enabled",
    "Enc",
    "Esp",
    "EveLog",
    "EventqueuePath",
    "EventqueueSlots",
    "Events",
    "Expect",
    "Expires",
    "Extended",
    "Extendedstatistics",
    "FailoverPeerip",
    "Fargw",
    "FetchCrls",
    "FileTags",
    "Files",
    "Filter1",
    "Filter2",
    "Firewall",
    "Firmware",
    "Flavour",
    "Format",
    "Forwarding",
    "From",
    "Fwrules",
    "Gateway",
    "GatewayItem",
    "Gateways1",
    "Gateways2",
    "Gatewayv6",
    "General",
    "Geoip",
    "Gid",
    "Gif",
    "Gifs",
    "Gre",
    "Gres",
    "Group",
    "Groupname",
    "Groups",
    "Ha",
    "HaPeers",
    "Hasync",
    "Hideidentity",
    "Hideversion",
    "Homenet",
    "Host",
    "Hostname",
    "Hosts",
    "Http",
    "HttpHost",
    "HttpPort",
    "HttpdAllow",
    "HttpdEnabled",
    "HttpdPassword",
    "HttpdPort",
    "HttpdUsername",
    "Ids",
    "If",
    "Ifgroups",
    "IgnoreAcquireTs",
    "Ike",
    "IkeName",
    "IkesaTableSegments",
    "IkesaTableSize",
    "Imc",
    "Imv",
    "InactiveTimeout",
    "Incomingnumtcp",
    "Infracachenumhosts",
    "Infrahostttl",
    "Infrakeepprobing",
    "InitLimitHalfOpen",
    "Insecuredomain",
    "InstallCrls",
    "Instance",
    "Instances",
    "Interface",
    "Interfaces1",
    "Interfaces2",
    "InternalDynamic",
    "Interval",
    "Ipaddr",
    "Ipaddrv6",
    "Ipprotocol",
    "Ips",
    "Ipsec",
    "Ipsecpsk",
    "Ipv6Allow",
    "Item",
    "Job",
    "Jobs",
    "Jostletimeout",
    "Kea",
    "Keepalive",
    "Key",
    "KeyPairs",
    "Knl",
    "Lagg",
    "Laggs",
    "Lan",
    "Language",
    "LbUseSticky",
    "Lib",
    "Lists",
    "Lo0",
    "LoadBalancer",
    "LocalZoneType",
    "Locals",
    "Lock",
    "LogLevel",
    "LogPayload",
    "Logfile",
    "Loglocal",
    "Loglocalactions",
    "Logqueries",
    "Logreplies",
    "Logservfail",
    "Logtagqueryreply",
    "Logverbosity",
    "Loopbacks",
    "Lvtemplate",
    "Mac",
    "Mailserver",
    "MakeBeforeBreak",
    "Match",
    "MaxIkev1Exchanges",
    "MaxUnackedClients",
    "Maxfilesize",
    "Maxpreserve",
    "Media",
    "Mediaopt",
    "Member",
    "Mgr",
    "MinProtocol",
    "MinProtocolDtls",
    "Mirror",
    "MmonitRegisterCredentials",
    "MmonitTimeout",
    "MmonitUrl",
    "Mode",
    "Monit",
    "MonitorDisable",
    "MonitorType",
    "Mpmalgo",
    "Msgcachesize",
    "Mtu",
    "Name",
    "Nat",
    "Neighbors",
    "Net",
    "Netflow",
    "Netflowbackup",
    "Network",
    "Networks",
    "Nextgid",
    "Nextuid",
    "Noarecords",
    "Noreglladdr6",
    "Noregrecords",
    "Noton",
    "Npt",
    "Ntpd",
    "Ntpserver",
    "Number",
    "Numberoptions",
    "Numqueriesperthread",
    "Nxdomain",
    "Onetoone",
    "OpenVpn1",
    "OpenVpnexport",
    "Openvpn2",
    "Opnsense1",
    "Opnsense2",
    "Optimization",
    "Options",
    "OtpSeed",
    "Outbound",
    "OutgoingInterface",
    "Outgoingnumtcp",
    "Outgoingrange",
    "Overwrites",
    "PassthroughNetworks",
    "Password",
    "Path",
    "Peers",
    "PfShareForward",
    "Pfsyncinterface",
    "Pfsyncpeerip",
    "Pfsyncversion",
    "Pidfile",
    "Pipes",
    "Plugins",
    "Policies",
    "Polltime",
    "Poolopts",
    "PooloptsSourcehashkey",
    "Pools",
    "Port",
    "PowerdAcMode",
    "PowerdBatteryMode",
    "PowerdNormalMode",
    "Ppp",
    "Ppps",
    "PreSharedKeys",
    "Prefer",
    "PreferredOldsa",
    "Prefetch",
    "Prefetchkey",
    "Priv",
    "Privateaddress",
    "Privatedomain",
    "Privkey",
    "Profile",
    "Promisc",
    "Protocol",
    "Prv",
    "Psk",
    "Pts",
    "Pubkey",
    "Qnameminstrict",
    "Queues",
    "Quick",
    "Range",
    "Reboot",
    "Recipient",
    "Refid",
    "Regdhcp",
    "Regdhcpdomain",
    "Regdhcpstatic",
    "Reminder",
    "Remotes",
    "Reservations",
    "RetransmitBase",
    "RetransmitJitter",
    "RetransmitLimit",
    "RetransmitTimeout",
    "RetransmitTries",
    "Revision",
    "Rocommunity",
    "Route",
    "Rrd",
    "Rrdbackup",
    "Rrsetcachesize",
    "Rule",
    "Rules",
    "Safesearch",
    "Scope",
    "Secret",
    "Send",
    "Sequence",
    "Serveexpired",
    "Serveexpiredclienttimeout",
    "Serveexpiredreplyttl",
    "Serveexpiredttl",
    "Serveexpiredttlreset",
    "Serveraddress",
    "Serverport",
    "Server",
    "Servers",
    "Service",
    "SessionResumption",
    "Snatrules",
    "Snmpd",
    "Source",
    "Sourceport",
    "Spds",
    "Spoofmac",
    "Ssh",
    "Ssl",
    "SslCertref",
    "Sslverify",
    "Sslversion",
    "Start",
    "Startdelay",
    "Starttimeout",
    "Statefile",
    "Statetype",
    "StaticKeys",
    "Staticmap",
    "Staticroutes",
    "Stats",
    "Stop",
    "StoreIntermediateCerts",
    "Subnet",
    "Subnets",
    "Subnetv6",
    "Subscription",
    "Swanctl",
    "Synchronizetoip",
    "Syncitems",
    "Syscontact",
    "Sysctl",
    "Syslocation",
    "Syslog1",
    "Syslog2",
    "SyslogEve",
    "System",
    "Tag",
    "Tagged",
    "Target",
    "Targets",
    "Templates",
    "Test",
    "Tests",
    "Theme",
    "ThisServerName",
    "Threads",
    "Time",
    "Timeout",
    "Timeservers",
    "Timezone",
    "Tls",
    "Tnc",
    "To",
    "ToclientGroups",
    "ToserverGroups",
    "Track6Interface",
    "Track6PrefixId",
    "TrafficShaper",
    "TriggerInitialWizard",
    "Trust",
    "Tunable",
    "Tunneladdress",
    "Txtsupport",
    "Type",
    "Uid",
    "Unbound",
    "Unboundplus",
    "Unwantedreplythreshold",
    "UpdateCron",
    "Updated",
    "Url",
    "User",
    "UserDefinedRules",
    "Username",
    "Usevirtualterminal",
    "ValidLifetime",
    "Valloglevel",
    "Value",
    "Verbosity",
    "Version",
    "Vip",
    "Virtual",
    "Virtualip",
    "Vlan",
    "Vlans",
    "Vtis",
    "Vxlans",
    "Wan",
    "Webgui",
    "Weight",
    "Whitelists",
    "Widgets",
    "Wildcards",
    "Winsserver",
    "Wireguard",
    "Wireless",
    "Zones",
]
