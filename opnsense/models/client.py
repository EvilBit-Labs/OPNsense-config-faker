from typing import Optional

from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.enabled import Enabled
from opnsense.models.keepalive import Keepalive
from opnsense.models.name import Name
from opnsense.models.psk import Psk
from opnsense.models.pubkey import Pubkey
from opnsense.models.serveraddress import Serveraddress
from opnsense.models.serverport import Serverport
from opnsense.models.tunneladdress import Tunneladdress

__NAMESPACE__ = "https://opnsense.org/config"


class Client(BaseModel):
    class Meta:
        name = "client"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    clients: Optional["Clients"] = field(
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
    pubkey: Pubkey | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    psk: Psk | None = field(
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
    serveraddress: Serveraddress | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    serverport: Serverport | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
    keepalive: Keepalive | None = field(
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


class Clients(BaseModel):
    class Meta:
        name = "clients"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    client: Client | None = field(
        default=None,
        metadata={
            "type": "Element",
            "namespace": "",
        },
    )
