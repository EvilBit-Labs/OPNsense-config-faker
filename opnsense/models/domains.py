from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Domains(BaseModel):
    class Meta:
        name = "domains"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
