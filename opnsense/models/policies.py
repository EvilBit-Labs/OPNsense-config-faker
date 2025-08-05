from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Policies(BaseModel):
    class Meta:
        name = "policies"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
