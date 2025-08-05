from typing import Optional

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.disableroutes import Disableroutes
from opnsense.models.dns import Dns
from opnsense.models.enabled import Enabled
from opnsense.models.gateway import Gateway
from opnsense.models.instance import Instance
from opnsense.models.mtu import Mtu
from opnsense.models.name import Name
from opnsense.models.peers import Peers
from opnsense.models.port import Port
from opnsense.models.privkey import Privkey
from opnsense.models.pubkey import Pubkey
from opnsense.models.tunneladdress import Tunneladdress

__NAMESPACE__ = "https://opnsense.org/config"


class Servers(BaseModel):
    class Meta:
        name = "servers"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    server: Optional["Server"] = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )


class Server(BaseModel):
    class Meta:
        name = "server"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    servers: Servers | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    enabled: Enabled | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    name: Name | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    instance: Instance | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    pubkey: Pubkey | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    privkey: Privkey | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    port: Port | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    mtu: Mtu | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    dns: Dns | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    tunneladdress: Tunneladdress | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    disableroutes: Disableroutes | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    gateway: Gateway | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    peers: Peers | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    uuid: object | None = field(
        default=None,
        metadata={
            "type": "Attribute",
        },
    )
    version: str | None = field(
        default=None,
        metadata={
            "type": "Attribute",
        },
    )
