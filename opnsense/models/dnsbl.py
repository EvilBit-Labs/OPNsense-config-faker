from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.address import Address
from opnsense.models.blocklists import Blocklists
from opnsense.models.enabled import Enabled
from opnsense.models.lists import Lists
from opnsense.models.nxdomain import Nxdomain
from opnsense.models.safesearch import Safesearch
from opnsense.models.type_mod import Type
from opnsense.models.whitelists import Whitelists
from opnsense.models.wildcards import Wildcards

__NAMESPACE__ = "https://opnsense.org/config"


class Dnsbl(BaseModel):
    class Meta:
        name = "dnsbl"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    enabled: Enabled = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    safesearch: Safesearch = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    type_value: Type = field(
        metadata={
            "name": "type",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    lists: Lists = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    whitelists: Whitelists = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    blocklists: Blocklists = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    wildcards: Wildcards = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    address: Address = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    nxdomain: Nxdomain = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
