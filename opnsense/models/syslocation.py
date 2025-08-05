from pydantic import BaseModel, ConfigDict

__NAMESPACE__ = "https://opnsense.org/config"


class Syslocation(BaseModel):
    class Meta:
        name = "syslocation"
        namespace = "https://opnsense.org/config"

    model_config = ConfigDict(defer_build=True)
