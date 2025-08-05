from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.advanced import Advanced
from opnsense.models.aliases import Aliases
from opnsense.models.default_action import DefaultAction
from opnsense.models.dnsbl import Dnsbl
from opnsense.models.domains import Domains
from opnsense.models.dots import Dots
from opnsense.models.enabled import Enabled
from opnsense.models.general import General
from opnsense.models.hosts import Hosts

__NAMESPACE__ = "https://opnsense.org/config"


class Unboundplus(BaseModel):
    class Meta:
        name = "unboundplus"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    general: General = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    advanced: Advanced = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    default_action: DefaultAction = field(
        metadata={
            "wrapper": "acls",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    dnsbl: Dnsbl = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    enabled: Enabled = field(
        metadata={
            "wrapper": "forwarding",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    dots: Dots = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    hosts: Hosts = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    aliases: Aliases = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    domains: Domains = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    version: str = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
