from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.npt import Npt
from opnsense.models.onetoone import Onetoone
from opnsense.models.rules import Rules
from opnsense.models.snatrules import Snatrules

__NAMESPACE__ = "https://opnsense.org/config"


class Filter1(BaseModel):
    class Meta:
        name = "Filter"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    rules: Rules = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    snatrules: Snatrules = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    npt: Npt = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    onetoone: Onetoone = field(
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
