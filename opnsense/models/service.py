from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.address import Address
from opnsense.models.depends import Depends
from opnsense.models.description import Description
from opnsense.models.enabled import Enabled
from opnsense.models.interface import Interface
from opnsense.models.match import Match
from opnsense.models.name import Name
from opnsense.models.path import Path
from opnsense.models.pidfile import Pidfile
from opnsense.models.polltime import Polltime
from opnsense.models.start import Start
from opnsense.models.starttimeout import Starttimeout
from opnsense.models.stop import Stop
from opnsense.models.tests import Tests
from opnsense.models.timeout import Timeout
from opnsense.models.type_mod import Type

__NAMESPACE__ = "https://opnsense.org/config"


class Service(BaseModel):
    class Meta:
        name = "service"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    enabled: Enabled = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    name: Name = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    description: Description = field(
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
    pidfile: Pidfile = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    match: Match = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    path: Path = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    timeout: Timeout = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    starttimeout: Starttimeout = field(
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
    interface: Interface = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    start: Start = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    stop: Stop = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    tests: Tests = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    depends: Depends = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    polltime: Polltime = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    uuid: object = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
