from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.children import Children
from opnsense.models.connections import Connections
from opnsense.models.locals import Locals
from opnsense.models.pools import Pools
from opnsense.models.remotes import Remotes
from opnsense.models.spds import Spds
from opnsense.models.vtis import Vtis

__NAMESPACE__ = "https://opnsense.org/config"


class Swanctl(BaseModel):
    class Meta:
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    connections: Connections = field(
        metadata={
            "name": "Connections",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    locals: Locals = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    remotes: Remotes = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    children: Children = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    pools: Pools = field(
        metadata={
            "name": "Pools",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    vtis: Vtis = field(
        metadata={
            "name": "VTIs",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    spds: Spds = field(
        metadata={
            "name": "SPDs",
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
