from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Noarecords(BaseModel):
    class Meta:
        name = "noarecords"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
