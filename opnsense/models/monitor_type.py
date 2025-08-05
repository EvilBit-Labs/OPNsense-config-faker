from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.descr import Descr
from opnsense.models.name import Name
from opnsense.models.options import Options
from opnsense.models.type_mod import Type

__NAMESPACE__ = "https://opnsense.org/config"


class MonitorType(BaseModel):
    class Meta:
        name = "monitor_type"
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
    descr: Descr = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    options: Options = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
