from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.alias import Alias
from opnsense.models.category_1 import Category1
from opnsense.models.filter_1 import Filter1
from opnsense.models.lvtemplate import Lvtemplate

__NAMESPACE__ = "https://opnsense.org/config"


class Firewall(BaseModel):
    class Meta:
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    lvtemplate: Lvtemplate = field(
        metadata={
            "name": "Lvtemplate",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    alias: Alias = field(
        metadata={
            "name": "Alias",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    category: Category1 = field(
        metadata={
            "name": "Category",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    filter: Filter1 = field(
        metadata={
            "name": "Filter",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
