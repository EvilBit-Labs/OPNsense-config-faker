from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.action import Action
from opnsense.models.condition import Condition
from opnsense.models.name import Name
from opnsense.models.path import Path
from opnsense.models.type_mod import Type

__NAMESPACE__ = "https://opnsense.org/config"


class Test(BaseModel):
    class Meta:
        name = "test"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    name: Name = field(
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
    condition: Condition = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    action: Action = field(
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
    uuid: object = field(
        metadata={
            "type": "Attribute",
            "required": True,
        }
    )
