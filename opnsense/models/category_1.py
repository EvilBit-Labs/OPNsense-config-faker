from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.categories import Categories

__NAMESPACE__ = "https://opnsense.org/config"


class Category1(BaseModel):
    class Meta:
        name = "Category"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    categories: Categories = field(
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
