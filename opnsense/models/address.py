from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Address(BaseModel):
    class Meta:
        name = "address"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
