use anchor_lang::prelude::*;

declare_id!("");

#[program]
pub mod farmacia {
    use super::*;

    //////////////////////////// Crear Farmacia /////////////////////////////////////

    pub fn crear_farmacia(context: Context<NuevaFarmacia>, nombre: String) -> Result<()> {

        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        let medicamentos: Vec<Medicamento> = Vec::new();

        context.accounts.farmacia.set_inner(Farmacia {
            owner: owner_id,
            nombre,
            medicamentos,
        });

        Ok(())
    }

    //////////////////////////// Agregar Medicamento /////////////////////////////////////

    pub fn agregar_medicamento(context: Context<NuevoMedicamento>, nombre: String, precio: u16) -> Result<()> {

        require!(
            context.accounts.farmacia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let medicamento = Medicamento {
            nombre,
            precio,
            disponible: true,
        };

        context.accounts.farmacia.medicamentos.push(medicamento);

        Ok(())
    }

    //////////////////////////// Eliminar Medicamento /////////////////////////////////////

    pub fn eliminar_medicamento(context: Context<NuevoMedicamento>, nombre: String) -> Result<()> {

        require!(
            context.accounts.farmacia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let medicamentos = &mut context.accounts.farmacia.medicamentos;

        for i in 0..medicamentos.len() {
            if medicamentos[i].nombre == nombre {
                medicamentos.remove(i);
                msg!("Medicamento {} eliminado!", nombre);
                return Ok(());
            }
        }

        Err(Errores::MedicamentoNoExiste.into())
    }

    //////////////////////////// Ver Medicamentos /////////////////////////////////////

    pub fn ver_medicamentos(context: Context<NuevoMedicamento>) -> Result<()> {

        require!(
            context.accounts.farmacia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        msg!("Lista de medicamentos: {:#?}", context.accounts.farmacia.medicamentos);

        Ok(())
    }

    //////////////////////////// Cambiar Disponibilidad /////////////////////////////////////

    pub fn alternar_estado(context: Context<NuevoMedicamento>, nombre: String) -> Result<()> {

        require!(
            context.accounts.farmacia.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let medicamentos = &mut context.accounts.farmacia.medicamentos;

        for i in 0..medicamentos.len() {

            let estado = medicamentos[i].disponible;

            if medicamentos[i].nombre == nombre {

                let nuevo_estado = !estado;

                medicamentos[i].disponible = nuevo_estado;

                msg!(
                    "El medicamento: {} ahora tiene disponibilidad: {}",
                    nombre,
                    nuevo_estado
                );

                return Ok(());
            }
        }

        Err(Errores::MedicamentoNoExiste.into())
    }
}

#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la farmacia")]
    NoEresElOwner,

    #[msg("Error, el medicamento no existe")]
    MedicamentoNoExiste,
}

#[account]
#[derive(InitSpace)]
pub struct Farmacia {

    owner: Pubkey,

    #[max_len(60)]
    nombre: String,

    #[max_len(10)]
    medicamentos: Vec<Medicamento>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Medicamento {

    #[max_len(60)]
    nombre: String,

    precio: u16,

    disponible: bool,
}

//////////////////////////// Contextos ////////////////////////////

#[derive(Accounts)]
pub struct NuevaFarmacia<'info> {

    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = Farmacia::INIT_SPACE + 8,
        seeds = [b"farmacia", owner.key().as_ref()],
        bump
    )]
    pub farmacia: Account<'info, Farmacia>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct NuevoMedicamento<'info> {

    pub owner: Signer<'info>,

    #[account(mut)]
    pub farmacia: Account<'info, Farmacia>,
}
