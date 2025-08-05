from pydantic import BaseModel, ConfigDict
from xsdata_pydantic.fields import field

from opnsense.models.charon import Charon
from opnsense.models.general import General
from opnsense.models.key_pairs import KeyPairs
from opnsense.models.pre_shared_keys import PreSharedKeys

__NAMESPACE__ = "https://opnsense.org/config"


class Ipsec(BaseModel):
    class Meta:
        name = "IPsec"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
    general: General = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    charon: Charon = field(
        metadata={
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    key_pairs: KeyPairs = field(
        metadata={
            "name": "keyPairs",
            "type": "Element",
            "namespace": "",
            "required": True,
        }
    )
    pre_shared_keys: PreSharedKeys = field(
        metadata={
            "name": "preSharedKeys",
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
